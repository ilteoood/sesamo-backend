import {HttpModule, Module} from '@nestjs/common';
import {OpenController} from "./controllers/open/OpenController";
import {FirestoreReader} from "./providers/FirestoreReader";
import {ActionHandlerFacade} from "./providers/ActionHandlerFacade";
import {CanOpen} from "./guards/CanOpen";
import {IftttHandler} from "./actionsHandlers/handlers/IftttHandler";
import {TestController} from "./controllers/open/TestController";
import {WarmUpController} from "./controllers/open/WarmUpController";

@Module({
    imports: [HttpModule],
    controllers: [OpenController, TestController, WarmUpController],
    providers: [FirestoreReader, ActionHandlerFacade, CanOpen, IftttHandler],
})
export class AppModule {
}
