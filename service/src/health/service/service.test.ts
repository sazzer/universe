import * as sut from './index';

describe('Health Service', () => {
  test('No healthchecks', async () => {
    const healthServuce = new sut.HealthChecker({});
    const result = await healthServuce.checkHealth();
    expect(result).toEqual({
      components: {}
    });
  });

  test('Passing healthcheck', async () => {
    const passingCheck = jest.fn();
    passingCheck.mockResolvedValue({});

    const healthServuce = new sut.HealthChecker({
      passing: {
        checkHealth: passingCheck
      }
    });

    const result = await healthServuce.checkHealth();

    expect(result).toEqual({
      components: {
        passing: {
          healthy: true
        }
      }
    });
    expect(passingCheck).toBeCalledTimes(1);
  });
  test('Failing healthcheck', async () => {
    const failingCheck = jest.fn();
    failingCheck.mockRejectedValue(new Error('Oops'));

    const healthServuce = new sut.HealthChecker({
      failing: {
        checkHealth: failingCheck
      }
    });

    const result = await healthServuce.checkHealth();

    expect(result).toEqual({
      components: {
        failing: {
          healthy: false,
          message: 'Oops'
        }
      }
    });
    expect(failingCheck).toBeCalledTimes(1);
  });
  test('Mixed healthchecks', async () => {
    const passingCheck = jest.fn();
    passingCheck.mockResolvedValue({});

    const failingCheck = jest.fn();
    failingCheck.mockRejectedValue(null);

    const healthServuce = new sut.HealthChecker({
      passing: {
        checkHealth: passingCheck
      },
      failing: {
        checkHealth: failingCheck
      }
    });

    const result = await healthServuce.checkHealth();

    expect(result).toEqual({
      components: {
        passing: {
          healthy: true
        },
        failing: {
          healthy: false
        }
      }
    });
    expect(passingCheck).toBeCalledTimes(1);
    expect(failingCheck).toBeCalledTimes(1);
  });
});
