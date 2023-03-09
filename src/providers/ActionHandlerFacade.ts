import {Injectable} from '@nestjs/common'

import {ActionBase} from '../actionsHandlers/ActionBase'
import {IftttHandler} from '../actionsHandlers/handlers/IftttHandler'
import {FirebaseServer} from '../models/firebase/FirebaseServer'

type ServerTypes = {
    [key: string]: ActionBase
}

@Injectable()
export class ActionHandlerFacade {

    private readonly serverTypes: ServerTypes

    constructor(private ifttt: IftttHandler) {
        this.serverTypes = {ifttt}
    }

    async open(firebaseServer: FirebaseServer, object: string): Promise<boolean> {
        const requestHandler = this.serverTypes[firebaseServer.type]
        return await requestHandler.open(firebaseServer.actions[object])
    }

}
