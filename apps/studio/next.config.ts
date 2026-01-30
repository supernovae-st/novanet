import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  // Enable experimental features
  experimental: {
    // Optimize package imports
    optimizePackageImports: ['lucide-react', '@xyflow/react', 'motion'],
  },

  // Turbopack configuration (Next.js 16+)
  turbopack: {},

  // Transpile novanet-core for imports
  transpilePackages: ['novanet-core'],

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
