import {ActionBase} from "../ActionBase";
import {HttpService, HttpStatus, Inject, Injectable, Logger} from "@nestjs/common";

@Injectable()
export class IftttHandler implements ActionBase {

    private readonly logger = new Logger(IftttHandler.name);

    @Inject()
    private http: HttpService;

    async open(configuration: Map<string, string>): Promise<boolean> {
        const urlToCall = this.buildUrl(configuration);
        this.logger.debug(`I'll call this IFTTT URL: ${urlToCall}`);
        const response = await this.http.post(urlToCall).toPromise();
        return response.status >= HttpStatus.OK && response.status < HttpStatus.AMBIGUOUS;
    }

    private buildUrl(configuration: any): string {
        return `https://maker.ifttt.com/trigger/${configuration.event}/with/key/${configuration.accessToken}`;
    }
}
