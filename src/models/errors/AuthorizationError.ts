import {HttpException, HttpStatus} from '@nestjs/common'

export class AuthorizationError extends HttpException {
    constructor(messageId) {
        super({messageId}, HttpStatus.UNAUTHORIZED)
    }
}
