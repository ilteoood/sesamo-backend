import {CanActivate, ExecutionContext, Injectable} from "@nestjs/common";
import {FirestoreReader} from "./FirestoreReader";
import {ActionHandlerFacade} from "./ActionHandlerFacade";
import {OpenRequest} from "../models/api/openRequest";
import {BusinessError} from "../models/errors/BusinessError";
import {FirebaseServer} from "../models/firebase/FirebaseServer";
import {AuthorizationError} from "../models/errors/AuthorizationError";

@Injectable()
export class CanOpen implements CanActivate {
    constructor(private firestoreReader: FirestoreReader, private actionHandlerFacade: ActionHandlerFacade) {
    }

    async canActivate(context: ExecutionContext): Promise<boolean> {
        const request = context.switchToHttp().getRequest();
        const requestBody = request.body;
        this.checkOpenRequest(requestBody);
        const firebaseServer = await this.firestoreReader.findServer(requestBody.serverId);
        this.checkFirebaseServer(firebaseServer);
        this.checkPermissions(firebaseServer, requestBody);
        const objectConfigurations = await this.firestoreReader.findConfigurations(requestBody.serverId, request.params.object);
        this.checkConfiguration(objectConfigurations);
        return true;
    }

    private checkOpenRequest(requestBody: OpenRequest) {
        if (!(requestBody.deviceId && requestBody.serverId)) {
            throw new BusinessError("wrong_request");
        }
    }

    private checkFirebaseServer(firebaseServer: FirebaseServer) {
        if (!firebaseServer) {
            throw new BusinessError("invalid_server");
        }
    }

    private checkPermissions(firebaseServer: FirebaseServer, requestBody: OpenRequest) {
        if (!firebaseServer.allowedDevices.includes(requestBody.deviceId)) {
            throw new AuthorizationError("unauthorized_device");
        }
    }

    private checkConfiguration(configuration: Map<string, string>) {
        if (!configuration || configuration.size == 0) {
            throw new BusinessError("invalid_action");
        }
    }
}
