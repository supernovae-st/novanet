import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  // Security headers
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          { key: 'X-Frame-Options', value: 'DENY' },
          { key: 'X-Content-Type-Options', value: 'nosniff' },
          { key: 'X-XSS-Protection', value: '1; mode=block' },
          { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
          {
            key: 'Content-Security-Policy',
            value: [
              "default-src 'self'",
              "script-src 'self' 'unsafe-eval' 'unsafe-inline'",
              "style-src 'self' 'unsafe-inline'",
              "img-src 'self' data: blob: https://pbs.twimg.com https://abs.twimg.com",
              "font-src 'self' data:",
              "connect-src 'self' bolt://localhost:7687 ws://localhost:* wss://localhost:*",
              "frame-ancestors 'none'",
            ].join('; '),
          },
          {
            key: 'Permissions-Policy',
            value: 'camera=(), microphone=(), geolocation=()',
          },
        ],
      },
    ];
  },

  // Enable experimental features
  experimental: {
    // Optimize package imports
    optimizePackageImports: ['lucide-react', '@xyflow/react', 'motion'],
  },

  // Turbopack configuration (Next.js 16+)
  turbopack: {},

  // Transpile @novanet/core for imports
  transpilePackages: ['@novanet/core'],

  // Webpack configuration for Neo4j driver
  webpack: (config, { isServer }) => {
    if (!isServer) {
      // Neo4j driver requires Node.js modules - mock them on client
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        net: false,
        tls: false,
        dns: false,
      };
    }
    return config;
  },
};

export default nextConfig;
