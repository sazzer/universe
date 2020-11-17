import * as sut from './model';

describe('SystemHealth', () => {
  describe('No components', () => {
    const systemHealth = new sut.SystemHealth({});

    test('healthy', () => {
      expect(systemHealth.healthy).toBe(true);
    });

    test('statusCode', () => {
      expect(systemHealth.statusCode()).toBe(200);
    });
  });

  describe('Passing component', () => {
    const systemHealth = new sut.SystemHealth({
      passing: {
        healthy: true
      }
    });

    test('healthy', () => {
      expect(systemHealth.healthy).toBe(true);
    });

    test('statusCode', () => {
      expect(systemHealth.statusCode()).toBe(200);
    });
  });
  describe('Failing component', () => {
    const systemHealth = new sut.SystemHealth({
      failing: {
        healthy: false
      }
    });

    test('healthy', () => {
      expect(systemHealth.healthy).toBe(false);
    });

    test('statusCode', () => {
      expect(systemHealth.statusCode()).toBe(503);
    });
  });
  describe('Mixed components', () => {
    const systemHealth = new sut.SystemHealth({
      passing: {
        healthy: true
      },
      failing: {
        healthy: false
      }
    });

    test('healthy', () => {
      expect(systemHealth.healthy).toBe(false);
    });

    test('statusCode', () => {
      expect(systemHealth.statusCode()).toBe(503);
    });
  });
});
