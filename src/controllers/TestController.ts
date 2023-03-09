import {Controller, Post, UseGuards} from '@nestjs/common'

import {CanOpen} from '../guards/CanOpen'
import {MessageResponse} from '../models/api/messageResponse'

@Controller('test')
export class TestController {

    private static readonly OK_MESSAGE: MessageResponse = new MessageResponse('test_ok')

    @Post(':object')
    @UseGuards(CanOpen)
    openObject(): MessageResponse {
        return TestController.OK_MESSAGE
    }

}
