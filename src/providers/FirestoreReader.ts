import {Injectable} from "@nestjs/common";
import {Firestore} from "@google-cloud/firestore";
import {FirebaseServer} from "../models/firebase/FirebaseServer";
import DocumentSnapshot = FirebaseFirestore.DocumentSnapshot;

@Injectable()
export class FirestoreReader {

    private fireStoreClient = new Firestore();
    private serversSnapshot: DocumentSnapshot;

    constructor() {
        this.fireStoreClient
            .doc('servers')
            .onSnapshot(documentSnapshot => this.serversSnapshot = documentSnapshot);
    }

    public findServer(serverId: string): undefined | FirebaseServer {
        const documentContent = this.serversSnapshot.get(serverId);
        return documentContent.exists ? FirebaseServer.convertDocument(documentContent) : undefined;
    }

    public findConfigurations(serverId: string, object: string): undefined | Map<string, string> {
        const configurationPath = `${serverId}/configurations/${object}`;
        const configurationContent = this.serversSnapshot.get(configurationPath);
        return configurationContent.exists ? this.documentConverter(configurationContent) : undefined;
    }

    public findAllowedDevices(serverId: string): string[] {
        const configurationPath = `servers/${serverId}/configurations/allowedDevices`;
        const configurationContent = this.serversSnapshot.get(configurationPath);
        return configurationContent.exists ? configurationContent.data().list : [];
    }

    private documentConverter(document: FirebaseFirestore.DocumentSnapshot): Map<string, string> {
        const mappedDocument = new Map();
        const documentData = document.data();
        const documentKeys = documentData ? Object.keys(documentData) : [];
        documentKeys.forEach(key => mappedDocument.set(key, documentData[key]));
        return mappedDocument;
    }

}
