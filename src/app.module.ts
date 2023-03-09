import {HttpModule} from '@nestjs/axios'
import {Module} from '@nestjs/common'

import {IftttHandler} from './actionsHandlers/handlers/IftttHandler'
import {OpenController} from './controllers/OpenController'
import {TestController} from './controllers/TestController'
import {WarmUpController} from './controllers/WarmUpController'
import {CanOpen} from './guards/CanOpen'
import {ActionHandlerFacade} from './providers/ActionHandlerFacade'
import {FirestoreReader} from './providers/FirestoreReader'

@Module({
    imports: [HttpModule],
    controllers: [OpenController, TestController, WarmUpController],
    providers: [FirestoreReader, ActionHandlerFacade, CanOpen, IftttHandler],
})
export class AppModule {
}
