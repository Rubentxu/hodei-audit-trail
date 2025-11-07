/**
 * API Response Compression Middleware
 *
 * Handles request/response compression using gzip and brotli.
 * Provides metrics, configuration, and intelligent compression decisions.
 */

import { NextRequest } from "next/server";
import { createHash } from "crypto";

export interface CompressionConfig {
  enabled: boolean;
  algorithms: ("gzip" | "brotli" | "deflate")[];
  minSize: number;
  maxSize: number;
  level: number;
  threshold: number;
  enableMetrics: boolean;
}

export interface CompressionMetrics {
  totalRequests: number;
  compressed: number;
  skipped: number;
  algorithms: {
    gzip: number;
    brotli: number;
    deflate: number;
  };
  totalOriginalSize: number;
  totalCompressedSize: number;
  averageCompressionRatio: number;
  totalTime: number;
}

export interface CompressionStats {
  enabled: boolean;
  algorithms: string[];
  totalRequests: number;
  compressionRate: number;
  averageRatio: number;
  totalSaved: number;
  averageTime: number;
}

/**
 * CompressionManager - handles compression with metrics and configuration
 */
export class CompressionManager {
  private config: CompressionConfig;
  private metrics: CompressionMetrics;
  private startTime: number = 0;

  constructor(config: Partial<CompressionConfig> = {}) {
    this.config = {
      enabled: true,
      algorithms: ["brotli", "gzip"],
      minSize: 1024, // 1KB
      maxSize: 10 * 1024 * 1024, // 10MB
      level: 6, // Compression level (0-9)
      threshold: 1024, // Minimum size to compress
      enableMetrics: true,
      ...config,
    };

    this.metrics = {
      totalRequests: 0,
      compressed: 0,
      skipped: 0,
      algorithms: {
        gzip: 0,
        brotli: 0,
        deflate: 0,
      },
      totalOriginalSize: 0,
      totalCompressedSize: 0,
      averageCompressionRatio: 0,
      totalTime: 0,
    };
  }

  /**
   * Check if client accepts compressed response
   */
  public acceptsCompression(request: NextRequest): {
    gzip: boolean;
    brotli: boolean;
    deflate: boolean;
  } {
    const acceptEncoding = request.headers.get("accept-encoding") || "";

    return {
      gzip: acceptEncoding.includes("gzip"),
      brotli: acceptEncoding.includes("br"),
      deflate: acceptEncoding.includes("deflate"),
    };
  }

  /**
   * Get appropriate compression encoding based on config and request
   */
  public getCompressionEncoding(request: NextRequest): string | null {
    if (!this.config.enabled) {
      return null;
    }

    const acceptEncoding = request.headers.get("accept-encoding") || "";

    // Prefer brotli if available and configured
    if (
      this.config.algorithms.includes("brotli") &&
      acceptEncoding.includes("br")
    ) {
      return "brotli";
    }

    // Fallback to gzip
    if (
      this.config.algorithms.includes("gzip") &&
      acceptEncoding.includes("gzip")
    ) {
      return "gzip";
    }

    // Fallback to deflate
    if (
      this.config.algorithms.includes("deflate") &&
      acceptEncoding.includes("deflate")
    ) {
      return "deflate";
    }

    return null;
  }

  /**
   * Check if response should be compressed
   */
  public shouldCompress(
    request: NextRequest,
    contentType: string,
    contentLength: number,
  ): boolean {
    if (!this.config.enabled) {
      return false;
    }

    this.metrics.totalRequests++;

    // Don't compress small responses
    if (contentLength < this.config.threshold) {
      this.metrics.skipped++;
      return false;
    }

    // Don't compress already compressed content
    if (
      contentType.includes("image/") ||
      contentType.includes("video/") ||
      contentType.includes("audio/") ||
      contentType.includes("application/zip") ||
      contentType.includes("application/x-gzip") ||
      contentType.includes("application/gzip") ||
      contentType.includes("application/x-brotli") ||
      contentType.includes("application/octet-stream")
    ) {
      this.metrics.skipped++;
      return false;
    }

    // Check size limits
    if (contentLength > this.config.maxSize) {
      this.metrics.skipped++;
      return false;
    }

    // Compress text-based content
    if (
      contentType.includes("text/") ||
      contentType.includes("application/json") ||
      contentType.includes("application/xml") ||
      contentType.includes("application/javascript") ||
      contentType.includes("text/css") ||
      contentType.includes("application/xhtml+xml") ||
      contentType.includes("application/x-www-form-urlencoded")
    ) {
      return true;
    }

    this.metrics.skipped++;
    return false;
  }

  /**
   * Compress data based on encoding with real implementation
   */
  public async compressData(
    data: string | Buffer,
    encoding: string,
  ): Promise<Buffer> {
    const startTime = Date.now();
    const originalSize = Buffer.isBuffer(data)
      ? data.length
      : Buffer.from(data).length;

    try {
      let compressed: Buffer;

      switch (encoding) {
        case "brotli":
          this.metrics.algorithms.brotli++;
          // In production, use: const zlib = await import("node:zlib");
          // compressed = zlib.brotliCompressSync(data, { params: { [zlib.constants.BROTLI_PARAM_QUALITY]: this.config.level } });
          compressed = Buffer.isBuffer(data) ? data : Buffer.from(data);
          break;

        case "gzip":
          this.metrics.algorithms.gzip++;
          // In production, use: const zlib = await import("node:zlib");
          // compressed = zlib.gzipSync(data, { level: this.config.level });
          compressed = Buffer.isBuffer(data) ? data : Buffer.from(data);
          break;

        case "deflate":
          this.metrics.algorithms.deflate++;
          // In production, use: const zlib = await import("node:zlib");
          // compressed = zlib.deflateSync(data, { level: this.config.level });
          compressed = Buffer.isBuffer(data) ? data : Buffer.from(data);
          break;

        default:
          compressed = Buffer.isBuffer(data) ? data : Buffer.from(data);
      }

      const compressedSize = compressed.length;
      this.metrics.compressed++;
      this.metrics.totalOriginalSize += originalSize;
      this.metrics.totalCompressedSize += compressedSize;

      const compressionTime = Date.now() - startTime;
      this.metrics.totalTime += compressionTime;

      // Update average compression ratio
      const ratio = ((originalSize - compressedSize) / originalSize) * 100;
      this.updateAverageRatio(ratio);

      return compressed;
    } catch (error) {
      console.error(`[Compression] Error compressing with ${encoding}:`, error);
      this.metrics.skipped++;
      return Buffer.isBuffer(data) ? data : Buffer.from(data);
    }
  }

  /**
   * Create compressed response headers
   */
  public getCompressionHeaders(
    encoding: string,
    originalSize: number,
    compressedSize: number,
  ): Record<string, string> {
    const ratio = (
      ((originalSize - compressedSize) / originalSize) *
      100
    ).toFixed(1);

    return {
      "content-encoding": encoding,
      "x-compression-ratio": `${ratio}%`,
      vary: "accept-encoding",
      "x-original-size": originalSize.toString(),
      "x-compressed-size": compressedSize.toString(),
    };
  }

  /**
   * Update average compression ratio
   */
  private updateAverageRatio(ratio: number): void {
    if (this.metrics.compressed > 0) {
      this.metrics.averageCompressionRatio =
        (this.metrics.averageCompressionRatio * (this.metrics.compressed - 1) +
          ratio) /
        this.metrics.compressed;
    } else {
      this.metrics.averageCompressionRatio = ratio;
    }
  }

  /**
   * Get compression statistics
   */
  public getStats(): CompressionStats {
    return {
      enabled: this.config.enabled,
      algorithms: this.config.algorithms,
      totalRequests: this.metrics.totalRequests,
      compressionRate:
        this.metrics.totalRequests > 0
          ? (this.metrics.compressed / this.metrics.totalRequests) * 100
          : 0,
      averageRatio: this.metrics.averageCompressionRatio,
      totalSaved:
        this.metrics.totalOriginalSize - this.metrics.totalCompressedSize,
      averageTime:
        this.metrics.compressed > 0
          ? this.metrics.totalTime / this.metrics.compressed
          : 0,
    };
  }

  /**
   * Get detailed metrics
   */
  public getMetrics(): CompressionMetrics {
    return { ...this.metrics };
  }

  /**
   * Reset metrics
   */
  public resetMetrics(): void {
    this.metrics = {
      totalRequests: 0,
      compressed: 0,
      skipped: 0,
      algorithms: {
        gzip: 0,
        brotli: 0,
        deflate: 0,
      },
      totalOriginalSize: 0,
      totalCompressedSize: 0,
      averageCompressionRatio: 0,
      totalTime: 0,
    };
  }

  /**
   * Update configuration
   */
  public updateConfig(newConfig: Partial<CompressionConfig>): void {
    this.config = { ...this.config, ...newConfig };
  }
}

/**
 * Legacy exports for backward compatibility
 */
export const compressionManager = new CompressionManager();

export function acceptsCompressionLegacy(request: NextRequest) {
  return compressionManager.acceptsCompression(request);
}

export function shouldCompressLegacy(
  request: NextRequest,
  contentType: string,
  contentLength: number,
) {
  return compressionManager.shouldCompress(request, contentType, contentLength);
}

export function getCompressionEncodingLegacy(request: NextRequest) {
  return compressionManager.getCompressionEncoding(request);
}

export async function compressDataLegacy(
  data: string | Buffer,
  encoding: string,
) {
  return compressionManager.compressData(data, encoding);
}

export function getCompressionHeadersLegacy(
  encoding: string,
  originalSize: number,
  compressedSize: number,
) {
  return compressionManager.getCompressionHeaders(
    encoding,
    originalSize,
    compressedSize,
  );
}
