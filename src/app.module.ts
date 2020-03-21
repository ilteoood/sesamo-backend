import {HttpModule, Module} from '@nestjs/common';
import {OpenController} from "./controllers/open/OpenController";
import {FirestoreReader} from "./providers/FirestoreReader";
import {ActionHandlerFacade} from "./providers/ActionHandlerFacade";
import {CanOpen} from "./providers/CanOpen";
import {IftttHandler} from "./actionsHandlers/handlers/IftttHandler";
import {TestController} from "./controllers/open/TestController";

@Module({
    imports: [HttpModule],
    controllers: [OpenController, TestController],
    providers: [FirestoreReader, ActionHandlerFacade, CanOpen, IftttHandler],
})
export class AppModule {
}
