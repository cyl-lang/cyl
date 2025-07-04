module.exports = {
  preset: "ts-jest",
  testEnvironment: "node",
  roots: ["<rootDir>/design/tests"],
  testMatch: ["**/*.test.ts"],
  collectCoverageFrom: ["design/src/**/*.ts", "!design/src/**/*.d.ts"],
  coverageDirectory: "coverage",
  coverageReporters: ["text", "lcov", "html"],
  moduleNameMapper: {
    "^chalk$": "<rootDir>/design/tests/__mocks__/chalk.ts",
  },
};
