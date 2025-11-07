// Tests for utility functions
describe('Utility Functions', () => {
  describe('String Utils', () => {
    it('should format strings correctly', () => {
      const formatString = (str: string) => {
        return str.trim().toLowerCase()
      }
      expect(formatString('  HELLO  ')).toBe('hello')
    })

    it('should validate emails', () => {
      const isValidEmail = (email: string) => {
        return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
      }
      expect(isValidEmail('test@example.com')).toBe(true)
      expect(isValidEmail('invalid-email')).toBe(false)
    })
  })

  describe('Date Utils', () => {
    it('should format dates', () => {
      const formatDate = (date: Date) => {
        return date.toISOString().split('T')[0]
      }
      const testDate = new Date('2025-01-01')
      expect(formatDate(testDate)).toBe('2025-01-01')
    })

    it('should calculate date differences', () => {
      const daysBetween = (date1: Date, date2: Date) => {
        const diff = Math.abs(date2.getTime() - date1.getTime())
        return Math.floor(diff / (1000 * 60 * 60 * 24))
      }
      const date1 = new Date('2025-01-01')
      const date2 = new Date('2025-01-05')
      expect(daysBetween(date1, date2)).toBe(4)
    })
  })

  describe('Array Utils', () => {
    it('should filter arrays', () => {
      const filterEven = (arr: number[]) => arr.filter(n => n % 2 === 0)
      expect(filterEven([1, 2, 3, 4, 5, 6])).toEqual([2, 4, 6])
    })

    it('should transform arrays', () => {
      const doubleArray = (arr: number[]) => arr.map(n => n * 2)
      expect(doubleArray([1, 2, 3])).toEqual([2, 4, 6])
    })
  })
})
