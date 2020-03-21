import {NestFactory} from '@nestjs/core';
import {AppModule} from './app.module';

async function bootstrap() {
    process.env["GOOGLE_APPLICATION_CREDENTIALS"] = "./firebase_reader.json";
    const app = await NestFactory.create(AppModule);
    await app.listen(process.env.PORT || 3000);
}

bootstrap();
