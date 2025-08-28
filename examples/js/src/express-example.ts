import express from 'express';
import cors from 'cors';
import helmet from 'helmet';
import morgan from 'morgan';
import { ScoutQuestClient, ServiceRegistrationOptions } from 'scoutquest-js';

interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  timestamp: string;
}

interface Task {
  id: number;
  title: string;
  description: string;
  completed: boolean;
  created_at: string;
}

interface HealthStatus {
  status: string;
  timestamp: string;
  details: {
    uptime: number;
    memory: NodeJS.MemoryUsage;
    version: string;
    port: number;
  };
}

class ExpressScoutQuestExample {
  private app: express.Application;
  private discoveryClient: ScoutQuestClient;
  private port: number;
  private serviceName = 'express-example-service';

  constructor(scoutquestUrl: string, port: number = 3001) {
    this.app = express();
    this.port = port;
    this.discoveryClient = new ScoutQuestClient(scoutquestUrl);
    
    this.setupMiddleware();
    this.setupRoutes();
  }

  private setupMiddleware(): void {
    // Security middleware
    this.app.use(helmet());
    
    // CORS middleware
    this.app.use(cors());
    
    // Logging middleware
    this.app.use(morgan('combined'));
    
    // Body parsing middleware
    this.app.use(express.json());
    this.app.use(express.urlencoded({ extended: true }));
  }

  private setupRoutes(): void {
    // Health check endpoint
    this.app.get('/health', this.healthCheck.bind(this));
    
    // Welcome endpoint
    this.app.get('/', this.welcome.bind(this));
    
    // Tasks endpoints (mock data)
    this.app.get('/api/tasks', this.getTasks.bind(this));
    this.app.post('/api/tasks', this.createTask.bind(this));
    this.app.get('/api/tasks/:id', this.getTask.bind(this));
    
    // Service discovery endpoints
    this.app.get('/api/services', this.getServices.bind(this));
    this.app.get('/api/services/:name', this.getServiceInstance.bind(this));
    
    // Service-to-service communication examples
    this.app.get('/api/call-user-service', this.callUserService.bind(this));
    this.app.get('/api/call-product-service', this.callProductService.bind(this));
    
    // Service monitoring
    this.app.get('/api/monitoring', this.getMonitoring.bind(this));
    
    // Error handling
    this.app.use(this.errorHandler.bind(this));
  }

  private successResponse<T>(data: T): ApiResponse<T> {
    return {
      success: true,
      data,
      timestamp: new Date().toISOString(),
    };
  }

  private errorResponse(message: string): ApiResponse {
    return {
      success: false,
      error: message,
      timestamp: new Date().toISOString(),
    };
  }

  private async healthCheck(req: express.Request, res: express.Response): Promise<void> {
    const healthStatus: HealthStatus = {
      status: 'Up',
      timestamp: new Date().toISOString(),
      details: {
        uptime: process.uptime(),
        memory: process.memoryUsage(),
        version: process.version,
        port: this.port,
      },
    };

    res.json(this.successResponse(healthStatus));
  }

  private async welcome(req: express.Request, res: express.Response): Promise<void> {
    const welcome = {
      message: 'Welcome to ScoutQuest Express.js Example Service!',
      service: this.serviceName,
      version: '1.0.0',
      endpoints: [
        'GET /health - Health check',
        'GET /api/tasks - List all tasks',
        'POST /api/tasks - Create a new task',
        'GET /api/tasks/:id - Get a specific task',
        'GET /api/services - List all services',
        'GET /api/services/:name - Get instance of a service',
        'GET /api/call-user-service - Call user service example',
        'GET /api/call-product-service - Call product service example',
        'GET /api/monitoring - Service monitoring stats',
      ],
    };

    res.json(this.successResponse(welcome));
  }

  private async getTasks(req: express.Request, res: express.Response): Promise<void> {
    const tasks: Task[] = [
      {
        id: 1,
        title: 'Learn ScoutQuest',
        description: 'Understand service discovery patterns',
        completed: true,
        created_at: new Date(Date.now() - 86400000).toISOString(),
      },
      {
        id: 2,
        title: 'Build microservice',
        description: 'Create a scalable microservice architecture',
        completed: false,
        created_at: new Date(Date.now() - 43200000).toISOString(),
      },
      {
        id: 3,
        title: 'Deploy to production',
        description: 'Deploy the service to production environment',
        completed: false,
        created_at: new Date().toISOString(),
      },
    ];

    res.json(this.successResponse(tasks));
  }

  private async createTask(req: express.Request, res: express.Response): Promise<void> {
    const { title, description } = req.body;

    if (!title) {
      res.status(400).json(this.errorResponse('Title is required'));
      return;
    }

    const newTask: Task = {
      id: Math.floor(Math.random() * 10000),
      title,
      description: description || '',
      completed: false,
      created_at: new Date().toISOString(),
    };

    console.log(`New task created: ${newTask.id}`);
    res.status(201).json(this.successResponse(newTask));
  }

  private async getTask(req: express.Request, res: express.Response): Promise<void> {
    const id = parseInt(req.params.id);

    if (id <= 3) {
      const task: Task = {
        id,
        title: `Task ${id}`,
        description: `Description of the task ${id}`,
        completed: id === 1,
        created_at: new Date(Date.now() - id * 86400000).toISOString(),
      };
      res.json(this.successResponse(task));
    } else {
      res.status(404).json(this.errorResponse('Task not found'));
    }
  }

  private async getServices(req: express.Request, res: express.Response): Promise<void> {
    try {
      const services = await this.discoveryClient.listServices();
      res.json(this.successResponse(services));
    } catch (error) {
      console.error('Error retrieving services:', error);
      res.status(500).json(this.errorResponse(`Error retrieving services: ${error}`));
    }
  }

  private async getServiceInstance(req: express.Request, res: express.Response): Promise<void> {
    try {
      const serviceName = req.params.name;
      const instance = await this.discoveryClient.discoverService(serviceName);
      res.json(this.successResponse(instance));
    } catch (error) {
      console.error(`Error discovering service ${req.params.name}:`, error);
      res.status(404).json(this.errorResponse(`Service ${req.params.name} not found: ${error}`));
    }
  }

  private async callUserService(req: express.Request, res: express.Response): Promise<void> {
    try {
      const users = await this.discoveryClient.get('user-service', '/api/users');
      const response = {
        message: 'Users retrieved from the user service',
        users,
      };
      res.json(this.successResponse(response));
    } catch (error) {
      console.error('Error calling the user service:', error);
      res.status(500).json(this.errorResponse(`User service not available: ${error}`));
    }
  }

  private async callProductService(req: express.Request, res: express.Response): Promise<void> {
    try {
      const products = await this.discoveryClient.get('product-service', '/api/products');
      const response = {
        message: 'Products retrieved from the product service',
        products,
      };
      res.json(this.successResponse(response));
    } catch (error) {
      console.error('Error calling the product service:', error);
      res.status(500).json(this.errorResponse(`Product service not available: ${error}`));
    }
  }

  private async getMonitoring(req: express.Request, res: express.Response): Promise<void> {
    try {
      const services = await this.discoveryClient.listServices();
      const serviceStats = [];

      for (const service of services) {
        try {
          const instance = await this.discoveryClient.discoverService(service.name);
          const isHealthy = instance.status === 'Up';

          serviceStats.push({
            name: service.name,
            instance: {
              id: instance.id,
              host: instance.host,
              port: instance.port,
              status: instance.status,
            },
            healthy: isHealthy,
            tags: service.tags,
          });
        } catch (error) {
          console.error(`Error discovering the service ${service.name}:`, error);
        }
      }

      const totalServices = services.length;
      const healthyServices = serviceStats.filter(s => s.healthy).length;

      const response = {
        microservices_count: totalServices,
        healthy_services: healthyServices,
        services: serviceStats,
      };

      res.json(this.successResponse(response));
    } catch (error) {
      console.error('Error retrieving microservices:', error);
      res.status(500).json(this.errorResponse(`Error retrieving microservices: ${error}`));
    }
  }

  private errorHandler(
    error: Error,
    req: express.Request,
    res: express.Response,
    next: express.NextFunction
  ): void {
    console.error('Unhandled error:', error);
    res.status(500).json(this.errorResponse('Internal server error'));
  }

  async start(): Promise<void> {
    try {
      // Register the service with ScoutQuest
      const registrationOptions: ServiceRegistrationOptions = {
        metadata: {
          version: '1.0.0',
          framework: 'express',
          language: 'typescript',
        },
        tags: ['api', 'example', 'express'],
      };

      await this.discoveryClient.registerService(
        this.serviceName,
        'localhost',
        this.port,
        registrationOptions
      );

      console.log(`✅ Service registered with ScoutQuest: ${this.serviceName}`);

      // Start the HTTP server
      this.app.listen(this.port, () => {
        console.log(`🚀 Express service started on port ${this.port}`);
        console.log(`📡 Health check: http://localhost:${this.port}/health`);
        console.log(`🏠 Welcome: http://localhost:${this.port}/`);
      });

      // Handle graceful shutdown
      process.on('SIGTERM', this.shutdown.bind(this));
      process.on('SIGINT', this.shutdown.bind(this));

    } catch (error) {
      console.error('Failed to start service:', error);
      process.exit(1);
    }
  }

  private async shutdown(): Promise<void> {
    console.log('🔄 Graceful shutdown initiated...');
    
    try {
      await this.discoveryClient.deregisterService();
      console.log('✅ Service deregistered from ScoutQuest');
    } catch (error) {
      console.error('Error during deregistration:', error);
    }

    process.exit(0);
  }
}

// Start the service if this file is run directly
if (require.main === module) {
  const scoutquestUrl = process.env.SCOUTQUEST_URL || 'http://localhost:8080';
  const port = parseInt(process.env.PORT || '3001', 10);

  const service = new ExpressScoutQuestExample(scoutquestUrl, port);
  service.start().catch(console.error);
}

export default ExpressScoutQuestExample;
