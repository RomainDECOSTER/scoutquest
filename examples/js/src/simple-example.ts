/**
 * Simple example showing basic ScoutQuest SDK usage
 */

import { ScoutQuestClient } from 'scoutquest-js';

async function simpleExample() {
  // Initialize the client
  const client = new ScoutQuestClient('http://localhost:8080');

  try {
    console.log('🚀 ScoutQuest Simple Example');
    console.log('============================\n');

    // 1. Register a service
    console.log('📝 Registering service...');
    const instance = await client.registerService(
      'simple-example-service',
      'localhost',
      3000,
      {
        tags: ['example', 'simple'],
        metadata: { version: '1.0.0' }
      }
    );
    console.log(`✅ Service registered with ID: ${instance.id}\n`);

    // 2. List all services
    console.log('📋 Listing all services...');
    const services = await client.listServices();
    console.log(`Found ${services.length} services:`);
    services.forEach(service => {
      console.log(`  - ${service.name}`);
    });
    console.log('');

    // 3. Discover a service
    console.log('🔍 Discovering our service...');
    const discoveredInstance = await client.discoverService('simple-example-service');
    console.log(`✅ Discovered service at: ${discoveredInstance.host}:${discoveredInstance.port}\n`);

    // 4. Clean up - deregister
    console.log('🧹 Deregistering service...');
    await client.deregisterService();
    console.log('✅ Service deregistered\n');

    console.log('🎉 Simple example completed successfully!');

  } catch (error) {
    console.error('❌ Error:', error);
  }
}

// Run if this file is executed directly
if (require.main === module) {
  simpleExample().catch(console.error);
}

export { simpleExample };
