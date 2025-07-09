module.exports = {
  testEnvironment: "node",
  roots: ["<rootDir>/compiler/dist"],
  testMatch: ["**/*.test.js"],
  collectCoverageFrom: ["compiler/dist/**/*.js", "!compiler/dist/**/*.d.ts"],
  coverageDirectory: "coverage",
  coverageReporters: ["text", "lcov", "html"],
  transform: {},
};
