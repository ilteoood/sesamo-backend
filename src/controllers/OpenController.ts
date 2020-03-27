import {Body, Controller, Param, Post, UseGuards} from "@nestjs/common";
import {FirestoreReader} from "../providers/FirestoreReader";
import {OpenRequest} from "../models/api/openRequest";
import {ActionHandlerFacade} from "../providers/ActionHandlerFacade";
import {CanOpen} from "../guards/CanOpen";
import {MessageResponse} from "../models/api/messageResponse";

@Controller('open')
export class OpenController {

    private static readonly OK_MESSAGE: MessageResponse = new MessageResponse("open_ok");

    constructor(private firestoreReader: FirestoreReader, private actionHandlerFacade: ActionHandlerFacade) {
    }

    @Post(':object')
    @UseGuards(CanOpen)
    async openObject(@Param('object') object, @Body() requestBody: OpenRequest): Promise<MessageResponse> {
        const firebaseServer = await this.firestoreReader.findServer(requestBody.serverId);
        await this.actionHandlerFacade.open(firebaseServer, object);
        return OpenController.OK_MESSAGE;
    }

}
