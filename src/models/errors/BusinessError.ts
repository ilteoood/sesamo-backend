import {HttpException, HttpStatus} from '@nestjs/common'

export class BusinessError extends HttpException {
    constructor(messageId) {
        super({messageId}, HttpStatus.INTERNAL_SERVER_ERROR)
    }
}
