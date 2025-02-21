import { useState, useEffect } from 'react'

interface BitcoinPrice {
  usd: number
  lastUpdated: Date
}

export function useBitcoinPrice() {
  const [price, setPrice] = useState<BitcoinPrice | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchPrice = async () => {
      try {
        const response = await fetch(
          'https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_last_updated_at=true'
        )
        if (!response.ok) throw new Error('Failed to fetch BTC price')
        const data = await response.json()
        
        setPrice({
          usd: data.bitcoin.usd,
          lastUpdated: new Date(data.bitcoin.last_updated_at * 1000)
        })
        setError(null)
      } catch (err) {
        setError('Failed to fetch Bitcoin price')
        console.error(err)
      } finally {
        setLoading(false)
      }
    }

    fetchPrice()
    // Refresh price every minute
    const interval = setInterval(fetchPrice, 60000)

    return () => clearInterval(interval)
  }, [])

  return { price, loading, error }
} 