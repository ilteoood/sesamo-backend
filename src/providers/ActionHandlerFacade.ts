import {Injectable} from '@nestjs/common'

import {ActionBase} from '../actionsHandlers/ActionBase'
import {HttpPostHandler} from '../actionsHandlers/handlers/HttpPostHandler'
import {IftttHandler} from '../actionsHandlers/handlers/IftttHandler'
import {FirebaseServer} from '../models/firebase/FirebaseServer'

type ServerTypes = 'ifttt' | 'httpPost'

type ServerHandlers = Record<ServerTypes, ActionBase>

@Injectable()
export class ActionHandlerFacade {

    private readonly serverTypes: ServerHandlers

    constructor(ifttt: IftttHandler, httpPost: HttpPostHandler) {
        this.serverTypes = {ifttt, httpPost}
    }

    async open(firebaseServer: FirebaseServer, object: string): Promise<boolean> {
        const requestHandler = this.serverTypes[firebaseServer.type]
        return await requestHandler.open(firebaseServer.actions[object])
    }

}
