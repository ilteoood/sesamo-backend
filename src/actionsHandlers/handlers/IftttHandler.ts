import {HttpService} from '@nestjs/axios'
import {Inject, Injectable, Logger} from '@nestjs/common'
import {firstValueFrom} from 'rxjs'

import {ActionBase} from '../ActionBase'

@Injectable()
export class IftttHandler extends ActionBase {

    private readonly logger = new Logger(IftttHandler.name)

    @Inject()
    private http: HttpService

    async open(configuration: Record<string, string>): Promise<boolean> {
        const urlToCall = this.buildUrl(configuration)
        this.logger.debug(`I'll call this IFTTT URL: ${urlToCall}`)
        const response = await firstValueFrom(this.http.post(urlToCall))
        return this.validateStatus(response)
    }

    private buildUrl(configuration: Record<string, string>): string {
        return `https://maker.ifttt.com/trigger/${configuration.event}/with/key/${configuration.accessToken}`
    }
}
