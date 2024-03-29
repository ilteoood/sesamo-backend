import {Body, Controller, Param, Post, UseGuards} from '@nestjs/common'

import {CanOpen} from '../guards/CanOpen'
import {MessageResponse} from '../models/api/messageResponse'
import {OpenRequest} from '../models/api/openRequest'
import {ActionHandlerFacade} from '../providers/ActionHandlerFacade'
import {FirestoreReader} from '../providers/FirestoreReader'

@Controller('open')
export class OpenController {

    private static readonly OK_MESSAGE: MessageResponse = new MessageResponse('open_ok')

    constructor(private firestoreReader: FirestoreReader, private actionHandlerFacade: ActionHandlerFacade) {
    }

    @Post(':object')
    @UseGuards(CanOpen)
    async openObject(@Param('object') object: string, @Body() requestBody: OpenRequest): Promise<MessageResponse> {
        const firebaseServer = this.firestoreReader.findServer(requestBody.serverId)
        await this.actionHandlerFacade.open(firebaseServer, object)
        return OpenController.OK_MESSAGE
    }

}
