/**
 * API Response Compression Middleware
 *
 * Handles request/response compression using gzip and brotli.
 */

import { NextRequest } from "next/server";

/**
 * Check if client accepts compressed response
 */
export function acceptsCompression(request: NextRequest): {
  gzip: boolean;
  br: boolean;
  deflate: boolean;
} {
  const acceptEncoding = request.headers.get("accept-encoding") || "";

  return {
    gzip: acceptEncoding.includes("gzip"),
    br: acceptEncoding.includes("br"),
    deflate: acceptEncoding.includes("deflate"),
  };
}

/**
 * Get appropriate compression encoding
 */
export function getCompressionEncoding(request: NextRequest): string | null {
  const acceptEncoding = request.headers.get("accept-encoding") || "";

  if (acceptEncoding.includes("br")) {
    return "br"; // Brotli (preferred)
  }
  if (acceptEncoding.includes("gzip")) {
    return "gzip"; // Gzip
  }
  if (acceptEncoding.includes("deflate")) {
    return "deflate";
  }

  return null;
}

/**
 * Check if response should be compressed
 */
export function shouldCompress(
  request: NextRequest,
  contentType: string,
  contentLength: number
): boolean {
  // Don't compress small responses
  if (contentLength < 1024) {
    return false;
  }

  // Don't compress already compressed content
  if (
    contentType.includes("image/") ||
    contentType.includes("video/") ||
    contentType.includes("audio/") ||
    contentType.includes("application/zip") ||
    contentType.includes("application/x-gzip") ||
    contentType.includes("application/gzip")
  ) {
    return false;
  }

  // Compress text-based content
  if (
    contentType.includes("text/") ||
    contentType.includes("application/json") ||
    contentType.includes("application/xml") ||
    contentType.includes("application/javascript") ||
    contentType.includes("text/css")
  ) {
    return true;
  }

  return false;
}

/**
 * Compress data based on encoding
 */
export async function compressData(
  data: string | Buffer,
  encoding: string
): Promise<Buffer> {
  switch (encoding) {
    case "br":
      // In a real implementation, use brotli compression
      // const brotli = await import("node:zlib").then(z => z.brotliCompress(data));
      // return brotli as Buffer;
      return Buffer.from(data);

    case "gzip":
      // In a real implementation, use gzip compression
      // const zlib = await import("node:zlib");
      // return zlib.gzipSync(data) as Buffer;
      return Buffer.from(data);

    case "deflate":
      // In a real implementation, use deflate compression
      // const zlib = await import("node:zlib");
      // return zlib.deflateSync(data) as Buffer;
      return Buffer.from(data);

    default:
      return Buffer.from(data);
  }
}

/**
 * Create compressed response headers
 */
export function getCompressionHeaders(
  encoding: string,
  originalSize: number,
  compressedSize: number
): Record<string, string> {
  const ratio = ((originalSize - compressedSize) / originalSize * 100).toFixed(1);

  return {
    "content-encoding": encoding,
    "x-compression-ratio": `${ratio}%`,
    "vary": "accept-encoding",
  };
}
