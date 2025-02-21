/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  images: {
    unoptimized: true,
  },
  // Since we're not using a custom domain yet
  basePath: '',
}

module.exports = nextConfig 