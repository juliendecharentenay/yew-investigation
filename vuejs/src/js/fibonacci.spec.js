import { fibonacci } from './fibonacci.js';

describe('fibonacci.js', () => {
  it('calculates 10th term of fibonacci', () => {
    expect(fibonacci(10)).toBe(55);
  });
});
