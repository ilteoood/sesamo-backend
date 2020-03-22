import {NestFactory} from '@nestjs/core';
import {AppModule} from './app.module';
import * as fs from "fs";

const FIREBASE_CREDENTIALS = "./firebase_reader.json";

async function bootstrap() {
    configureCredentials();
    const app = await NestFactory.create(AppModule);
    await app.listen(process.env.PORT || 3000);
}

function configureCredentials() {
    if (fs.existsSync(FIREBASE_CREDENTIALS)) {
        process.env.GOOGLE_APPLICATION_CREDENTIALS = FIREBASE_CREDENTIALS;
    }
}

bootstrap();
