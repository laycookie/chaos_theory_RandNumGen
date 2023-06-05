const path = require("path");
const sass = require("sass");

/** @type {import('next').NextConfig} */
const nextConfig = {
  sassOptions: {
    implementation: sass,
    includePaths: [path.resolve(__dirname, "node_modules")],
  },
};

module.exports = nextConfig;
