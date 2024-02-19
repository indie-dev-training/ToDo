/** @type {import('next').NextConfig} */

const withBundleAnalyzer = require("@next/bundle-analyzer")({
  enabled: process.env.ANALYZE === "true"
});

const nextConfig = {
  reactStrictMode: false,
  experimental: {
    webpackBuildWorker: false
  }
};

module.exports = withBundleAnalyzer({ ...nextConfig });
