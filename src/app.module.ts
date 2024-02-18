import {HttpModule} from '@nestjs/axios'
import {Module} from '@nestjs/common'
import {HttpPostHandler} from 'src/actionsHandlers/handlers/HttpPostHandler'
import {IftttHandler} from 'src/actionsHandlers/handlers/IftttHandler'
import {OpenController} from 'src/controllers/OpenController'
import {TestController} from 'src/controllers/TestController'
import {WarmUpController} from 'src/controllers/WarmUpController'
import {CanOpen} from 'src/guards/CanOpen'
import {ActionHandlerFacade} from 'src/providers/ActionHandlerFacade'
import {FirestoreReader} from 'src/providers/FirestoreReader'

@Module({
    imports: [HttpModule],
    controllers: [OpenController, TestController, WarmUpController],
    providers: [FirestoreReader, ActionHandlerFacade, CanOpen, IftttHandler, HttpPostHandler],
})
export class AppModule {
}
