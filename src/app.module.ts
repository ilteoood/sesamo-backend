import {HttpModule, Module} from '@nestjs/common';
import {OpenController} from "./controllers/OpenController";
import {FirestoreReader} from "./providers/FirestoreReader";
import {ActionHandlerFacade} from "./providers/ActionHandlerFacade";
import {CanOpen} from "./guards/CanOpen";
import {IftttHandler} from "./actionsHandlers/handlers/IftttHandler";
import {TestController} from "./controllers/TestController";
import {WarmUpController} from "./controllers/WarmUpController";

@Module({
    imports: [HttpModule],
    controllers: [OpenController, TestController, WarmUpController],
    providers: [FirestoreReader, ActionHandlerFacade, CanOpen, IftttHandler],
})
export class AppModule {
}
