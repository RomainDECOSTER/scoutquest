import * as SDK from '../index';
import { ScoutQuestClient } from '../client';

describe('SDK Index', () => {
  describe('exports', () => {
    it('should export ScoutQuestClient', () => {
      expect(SDK.ScoutQuestClient).toBeDefined();
      expect(SDK.ScoutQuestClient).toBe(ScoutQuestClient);
    });

    it('should export LoadBalancer', () => {
      expect(SDK.LoadBalancer).toBeDefined();
    });

    it('should export error classes', () => {
      expect(SDK.ScoutQuestError).toBeDefined();
      expect(SDK.isScoutQuestError).toBeDefined();
    });

    it('should export types and enums', () => {
      expect(SDK.LoadBalancingStrategy).toBeDefined();
      expect(SDK.InstanceStatus).toBeDefined();
    });

    it('should export VERSION constant', () => {
      expect(SDK.VERSION).toBe('1.0.0');
    });

    it('should have default export as ScoutQuestClient', () => {
      expect(SDK.default).toBe(ScoutQuestClient);
    });
  });

  describe('createClient', () => {
    it('should create a new client instance', () => {
      const client = SDK.createClient('http://localhost:8080');
      expect(client).toBeInstanceOf(ScoutQuestClient);
    });

    it('should create client with config', () => {
      const config = { timeout: 60000 };
      const client = SDK.createClient('http://localhost:8080', config);
      expect(client).toBeInstanceOf(ScoutQuestClient);
    });
  });
});
