import {Body, Controller, Param, Post, UseGuards} from "@nestjs/common";
import {MessageResponse} from "../../models/api/messageResponse";
import {CanOpen} from "../../providers/CanOpen";
import {OpenRequest} from "../../models/api/openRequest";

@Controller('test')
export class TestController {

    private static readonly OK_MESSAGE: MessageResponse = new MessageResponse("test_ok");

    @Post(':object')
    @UseGuards(CanOpen)
    openObject(@Param('object') object, @Body() requestBody: OpenRequest): MessageResponse {
        return TestController.OK_MESSAGE;
    }

}
