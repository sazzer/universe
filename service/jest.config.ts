export default {
  clearMocks: true,
  collectCoverage: true,
  coverageDirectory: 'target/coverage',
  coverageProvider: 'v8',
  preset: 'ts-jest',
  testEnvironment: 'node',
  verbose: true,
  errorOnDeprecated: true,
  notify: true
};
