module.exports = {
  testEnvironment: "node",
  roots: ["<rootDir>/design/tests"],
  testMatch: ["**/*.test.ts", "**/*.test.mts"],
  collectCoverageFrom: ["design/src/**/*.ts", "!design/src/**/*.d.ts"],
  coverageDirectory: "coverage",
  coverageReporters: ["text", "lcov", "html"],
  transform: {
    "^.+\\.(ts|mts)$": "babel-jest",
  },
  moduleNameMapper: {
    "^@src/(.*)$": "<rootDir>/design/src/$1",
    // Map ESM .js extension imports to .ts files for internal modules
    // Only map .js to .ts for internal design/src imports
  },
  verbose: true,
  extensionsToTreatAsEsm: [".ts", ".mts"],
};
