module.exports = {
  testEnvironment: "node",
  roots: ["<rootDir>/dist"],
  testMatch: ["**/*.test.js"],
  collectCoverageFrom: ["dist/**/*.js", "!dist/**/*.d.ts"],
  coverageDirectory: "coverage",
  coverageReporters: ["text", "lcov", "html"],
  transform: {}
};
