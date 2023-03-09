import {HttpService} from '@nestjs/axios'
import {HttpStatus, Inject, Injectable, Logger} from '@nestjs/common'
import {firstValueFrom} from 'rxjs'

import {ActionBase} from '../ActionBase'

@Injectable()
export class IftttHandler implements ActionBase {

    private readonly logger = new Logger(IftttHandler.name)

    @Inject()
    private http: HttpService

    async open(configuration: Map<string, string>): Promise<boolean> {
        const urlToCall = this.buildUrl(configuration)
        this.logger.debug(`I'll call this IFTTT URL: ${urlToCall}`)
        const response = await firstValueFrom(this.http.post(urlToCall))
        return response.status >= HttpStatus.OK && response.status < HttpStatus.AMBIGUOUS
    }

    private buildUrl(configuration: Map<string, string>): string {
        return `https://maker.ifttt.com/trigger/${configuration.get('event')}/with/key/${configuration.get('accessToken')}`
    }
}
