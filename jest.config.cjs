module.exports = {
  testEnvironment: "node",
  roots: ["<rootDir>/compiler/src", "<rootDir>/design/tests"],
  testMatch: ["**/*.test.js", "**/*.test.ts", "**/*.test.mts"],
  collectCoverageFrom: ["compiler/src/**/*.js", "!compiler/src/**/*.d.ts"],
  coverageDirectory: "coverage",
  coverageReporters: ["text", "lcov", "html"],
  transform: {
    "^.+\\.(ts|mts|js)$": ["ts-jest", { useESM: true }],
  },
  extensionsToTreatAsEsm: [".ts", ".mts"],
  globals: {
    'ts-jest': {
      useESM: true,
      tsconfig: 'tsconfig.json',
    },
  },
  moduleNameMapper: {},
  verbose: true,
};
