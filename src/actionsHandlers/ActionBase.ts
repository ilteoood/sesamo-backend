import {HttpStatus} from '@nestjs/common'
import {AxiosResponse} from 'axios'

export abstract class ActionBase {
    abstract open(configuration: Record<string, string>): Promise<boolean>;

    protected validateStatus(response: AxiosResponse): boolean {
        return response.status >= HttpStatus.OK && response.status < HttpStatus.AMBIGUOUS
    }
}
