module.exports = {
  testEnvironment: "node",
  roots: ["<rootDir>/compiler/src"],
  testMatch: ["**/*.test.js"],
  collectCoverageFrom: ["compiler/src/**/*.js", "!compiler/src/**/*.d.ts"],
  coverageDirectory: "coverage",
  coverageReporters: ["text", "lcov", "html"],
  transform: {},
  verbose: true,
};
