// Jest setup file
beforeEach(() => {
  // Clear all mocks before each test
  jest.clearAllMocks();
  // Use real timers by default to avoid blocking retries
  jest.useRealTimers();
});

afterEach(() => {
  // Clean up any timers or intervals
  jest.clearAllTimers();
  jest.useRealTimers();
});

// Mock console methods to reduce noise in tests
global.console = {
  ...console,
  // Keep native behaviour for log and info
  log: jest.fn(),
  debug: jest.fn(),
  info: jest.fn(),
  warn: jest.fn(),
  error: jest.fn(),
};
