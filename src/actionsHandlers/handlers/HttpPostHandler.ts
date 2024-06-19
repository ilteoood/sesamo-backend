import {HttpService} from '@nestjs/axios'
import {Inject, Injectable, Logger} from '@nestjs/common'
import {firstValueFrom} from 'rxjs'
import FormData from 'form-data'

import {ActionBase} from '../ActionBase'

@Injectable()
export class HttpPostHandler extends ActionBase {

    private readonly logger = new Logger(HttpPostHandler.name)

    @Inject()
    private http: HttpService

    async open(configuration: Record<string, string>): Promise<boolean> {
        const urlToCall = this.buildUrl(configuration)
        this.logger.debug(`I'll call this URL: ${urlToCall}`)
        const response = await firstValueFrom(this.http.post(urlToCall, this.buildBody(configuration)))
        return this.validateStatus(response)
    }

    private buildUrl(configuration: Record<string, string>): string {
        return configuration.url
    }

    private buildBody(configuration: Record<string, string>): FormData {
        const formData = new FormData();
        
        new URLSearchParams(configuration.body).forEach((value, key) => formData.append(key, value));

        return formData;
    }
}
