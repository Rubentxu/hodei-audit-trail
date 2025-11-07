import { colors } from './colors'
import { typography } from './typography'
import { spacing } from './spacing'
import { borderRadius } from './border-radius'

export const designTokens = {
  colors,
  typography,
  spacing,
  borderRadius,
}

export type DesignTokens = typeof designTokens
