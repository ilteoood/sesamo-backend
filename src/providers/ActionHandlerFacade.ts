import {Injectable} from "@nestjs/common";
import {FirebaseServer} from "../models/firebase/FirebaseServer";
import {ActionBase} from "../actionsHandlers/ActionBase";
import {IftttHandler} from "../actionsHandlers/handlers/IftttHandler";

type ServerTypes = {
    [key: string]: ActionBase
}

@Injectable()
export class ActionHandlerFacade {

    private readonly serverTypes: ServerTypes;

    constructor(private ifttt: IftttHandler) {
        this.serverTypes = {ifttt};
    }

    async open(firebaseServer: FirebaseServer, configurations: Map<string, string>): Promise<boolean> {
        const requestHandler = this.serverTypes[firebaseServer.type];
        return await requestHandler.open(configurations);
    }

}
