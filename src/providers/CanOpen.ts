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
        this.checkObject(firebaseServer, request.params.object);
        this.checkPermissions(firebaseServer, requestBody);
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

    private checkObject(firebaseServer: FirebaseServer, object: string) {
        if (!firebaseServer.configurations[object]) {
            throw new BusinessError("invalid_action");
        }
    }

    private checkPermissions(firebaseServer: FirebaseServer, requestBody: OpenRequest) {
        if (!firebaseServer.allowedDevices.includes(requestBody.deviceId)) {
            throw new AuthorizationError("unauthorized_device");
        }
    }
}
