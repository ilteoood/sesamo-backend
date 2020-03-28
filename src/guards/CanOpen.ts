import {CanActivate, ExecutionContext, Injectable} from "@nestjs/common";
import {FirestoreReader} from "../providers/FirestoreReader";
import {OpenRequest} from "../models/api/openRequest";
import {BusinessError} from "../models/errors/BusinessError";
import {AuthorizationError} from "../models/errors/AuthorizationError";

@Injectable()
export class CanOpen implements CanActivate {
    constructor(private firestoreReader: FirestoreReader) {
    }

    canActivate(context: ExecutionContext): boolean {
        const request = context.switchToHttp().getRequest();
        const requestBody = request.body;
        this.checkOpenRequest(requestBody);
        this.checkFirebaseServer(requestBody);
        this.checkConfiguration(requestBody, request.params);
        this.checkPermissions(requestBody);
        return true;
    }

    private checkOpenRequest(requestBody: OpenRequest) {
        if (!(requestBody.deviceId && requestBody.serverId)) {
            throw new BusinessError("wrong_request");
        }
    }

    private checkFirebaseServer(requestBody: OpenRequest) {
        const firebaseServer = this.firestoreReader.findServer(requestBody.serverId);
        if (!firebaseServer) {
            throw new BusinessError("invalid_server");
        }
    }

    private checkConfiguration(requestBody: OpenRequest, requestParams: any) {
        const objectConfigurations = this.firestoreReader.findConfigurations(requestBody.serverId, requestParams.object);
        if (!objectConfigurations || objectConfigurations.size == 0) {
            throw new BusinessError("invalid_action");
        }
    }

    private checkPermissions(requestBody: OpenRequest) {
        const allowedDevices = this.firestoreReader.findAllowedDevices(requestBody.serverId);
        if (!allowedDevices.includes(requestBody.deviceId)) {
            throw new AuthorizationError("unauthorized_device");
        }
    }
}
