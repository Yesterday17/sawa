import type { Price } from '../client/types.gen'
import { client } from '../client/client.gen'

export function getImageUrl(mediaId: string) {
  const baseUrl = client.getConfig().baseUrl
  return `${baseUrl}/media/${mediaId}`
}

export function formatPrice(price: Price): string {
  const { amount, currency } = price

  // Runtime check for string currency (just in case)
  if (typeof currency === 'string') {
    return `${amount} ${currency}`
  }

  const value = amount / 10 ** currency.exponent

  // Format the number part
  const formattedValue = value.toLocaleString(undefined, {
    minimumFractionDigits: currency.exponent,
    maximumFractionDigits: currency.exponent,
  })

  return `${currency.symbol}${formattedValue}`
}
