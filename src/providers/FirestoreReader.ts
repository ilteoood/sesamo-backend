import {Injectable} from "@nestjs/common";
import {Firestore} from "@google-cloud/firestore";
import {FirebaseServer} from "../models/firebase/FirebaseServer";

@Injectable()
export class FirestoreReader {

    private fireStoreClient = new Firestore();

    public async findServer(serverId: string): Promise<undefined | FirebaseServer> {
        const serverPath = `servers/${serverId}`;
        const serverDocument = this.fireStoreClient.doc(serverPath);
        const documentContent = await serverDocument.get();
        return documentContent.exists ? FirebaseServer.convertDocument(documentContent) : undefined;
    }

}
