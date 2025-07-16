module.exports = {
  testEnvironment: "node",
  roots: ["<rootDir>/compiler/src", "<rootDir>/design/tests"],
  testMatch: ["**/*.test.js", "**/*.test.ts", "**/*.test.mts"],
  collectCoverageFrom: ["compiler/src/**/*.js", "!compiler/src/**/*.d.ts"],
  coverageDirectory: "coverage",
  coverageReporters: ["text", "lcov", "html"],
  transform: {},
  verbose: true,
};
