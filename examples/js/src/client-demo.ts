import { ScoutQuestClient, ServiceRegistrationOptions } from 'scoutquest-js';

interface User {
  id: number;
  name: string;
  email: string;
}

interface Product {
  id: number;
  name: string;
  price: number;
  category: string;
}

class ClientDemo {
  private client: ScoutQuestClient;

  constructor(scoutquestUrl: string) {
    this.client = new ScoutQuestClient(scoutquestUrl);
  }

  async demonstrateServiceDiscovery(): Promise<void> {
    console.log('🔍 === Service Discovery Demo ===');

    try {
      // List all available services
      console.log('\n📋 Listing all services:');
      const services = await this.client.listServices();
      console.log(`Found ${services.length} services:`);
      services.forEach((service: any) => {
        console.log(`  - ${service.name} (tags: ${service.tags.join(', ')})`);
      });

      // Discover a specific service
      if (services.length > 0) {
        const serviceName = services[0].name;
        console.log(`\n🎯 Discovering service: ${serviceName}`);

        try {
          const instance = await this.client.discoverService(serviceName);
          console.log('Service instance found:');
          console.log(`  - ID: ${instance.id}`);
          console.log(`  - Host: ${instance.host}:${instance.port}`);
          console.log(`  - Status: ${instance.status}`);
          console.log(`  - URL: ${instance.secure ? 'https' : 'http'}://${instance.host}:${instance.port}`);
          console.log(`  - Tags: ${instance.tags.join(', ')}`);
        } catch (error) {
          console.log(`❌ Service ${serviceName} not available: ${error}`);
        }
      }

    } catch (error) {
      console.error('Error during service discovery:', error);
    }
  }

  async demonstrateServiceCalls(): Promise<void> {
    console.log('\n📞 === Service Communication Demo ===');

    // Try to call user service
    console.log('\n👥 Calling user service:');
    try {
      const users = await this.client.get<User[]>('user-service', '/api/users');
      console.log(`✅ Retrieved ${users.length} users:`);
      users.slice(0, 3).forEach((user: any) => {
        console.log(`  - ${user.name} (${user.email})`);
      });
    } catch (error) {
      console.log(`❌ User service call failed: ${error}`);
    }

    // Try to call product service
    console.log('\n📦 Calling product service:');
    try {
      const products = await this.client.get<Product[]>('product-service', '/api/products');
      console.log(`✅ Retrieved ${products.length} products:`);
      products.slice(0, 3).forEach((product: any) => {
        console.log(`  - ${product.name}: $${product.price} (${product.category})`);
      });
    } catch (error) {
      console.log(`❌ Product service call failed: ${error}`);
    }

    // Try to create a new user
    console.log('\n➕ Creating a new user:');
    try {
      const newUser = {
        name: 'John Demo',
        email: 'john.demo@example.com',
        age: 30,
      };

      const createdUser = await this.client.post<User>('user-service', '/api/users', newUser);
      console.log('✅ User created successfully:');
      console.log(`  - ID: ${createdUser.id}`);
      console.log(`  - Name: ${createdUser.name}`);
      console.log(`  - Email: ${createdUser.email}`);
    } catch (error) {
      console.log(`❌ User creation failed: ${error}`);
    }
  }

  async demonstrateServiceRegistration(): Promise<void> {
    console.log('\n📝 === Service Registration Demo ===');

    try {
      const serviceName = 'demo-client-service';
      const registrationOptions: ServiceRegistrationOptions = {
        metadata: {
          version: '1.0.0',
          type: 'demo',
          language: 'typescript',
        },
        tags: ['demo', 'client', 'example'],
      };

      console.log(`\n📋 Registering service: ${serviceName}`);
      const instance = await this.client.registerService(
        serviceName,
        'localhost',
        9999,
        registrationOptions
      );

      console.log('✅ Service registered successfully:');
      console.log(`  - ID: ${instance.id}`);
      console.log(`  - Service: ${instance.service_name}`);
      console.log(`  - Address: ${instance.host}:${instance.port}`);
      console.log(`  - Status: ${instance.status}`);

      // Wait a bit
      console.log('\n⏳ Waiting 3 seconds...');
      await new Promise(resolve => setTimeout(resolve, 3000));

      // Deregister the service
      console.log('\n📤 Deregistering service...');
      await this.client.deregisterService();
      console.log('✅ Service deregistered successfully');

    } catch (error) {
      console.error('Error during service registration demo:', error);
    }
  }

  async demonstrateServicesByTag(): Promise<void> {
    console.log('\n🏷️ === Services by Tag Demo ===');

    try {
      // Get all services and filter by tags
      console.log('\n🔍 Getting all services and filtering by tags...');
      const allServices = await this.client.listServices();

      const tags = ['api', 'web', 'database', 'cache'];

      for (const tag of tags) {
        const servicesWithTag = allServices.filter((service: any) =>
          service.tags.includes(tag)
        );

        if (servicesWithTag.length > 0) {
          console.log(`✅ Found ${servicesWithTag.length} service(s) with tag '${tag}':`);
          servicesWithTag.forEach((service: any) => {
            console.log(`  - ${service.name} (tags: ${service.tags.join(', ')})`);
          });
        } else {
          console.log(`ℹ️  No services found with tag: ${tag}`);
        }
      }
    } catch (error) {
      console.error('Error during services by tag demo:', error);
    }
  }

  async demonstrateErrorHandling(): Promise<void> {
    console.log('\n❌ === Error Handling Demo ===');

    // Try to discover a non-existent service
    console.log('\n🔍 Trying to discover non-existent service:');
    try {
      await this.client.discoverService('non-existent-service');
      console.log('✅ This should not happen');
    } catch (error) {
      console.log(`✅ Expected error caught: ${error}`);
    }

    // Try to call a non-existent endpoint
    console.log('\n📞 Trying to call non-existent endpoint:');
    try {
      await this.client.get('user-service', '/api/non-existent');
      console.log('✅ This should not happen');
    } catch (error) {
      console.log(`✅ Expected error caught: ${error}`);
    }
  }

  async runAllDemos(): Promise<void> {
    console.log('🎪 Starting ScoutQuest Client Demo');
    console.log('=====================================\n');

    try {
      await this.demonstrateServiceDiscovery();
      await this.demonstrateServiceCalls();
      await this.demonstrateServicesByTag();
      await this.demonstrateServiceRegistration();
      await this.demonstrateErrorHandling();

      console.log('\n🎉 Demo completed successfully!');
    } catch (error) {
      console.error('❌ Demo failed:', error);
    }
  }
}

// Run the demo if this file is executed directly
async function main() {
  const scoutquestUrl = process.env.SCOUTQUEST_URL || 'http://localhost:8080';
  console.log(`🔗 Connecting to ScoutQuest at: ${scoutquestUrl}\n`);

  const demo = new ClientDemo(scoutquestUrl);
  await demo.runAllDemos();
}

if (require.main === module) {
  main().catch(console.error);
}

export default ClientDemo;
