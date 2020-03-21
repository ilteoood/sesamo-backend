import {Injectable} from "@nestjs/common";
import {Firestore} from "@google-cloud/firestore";
import {FirebaseServer} from "../models/firebase/FirebaseServer";

@Injectable()
export class FirestoreReader {

    private fireStoreClient = new Firestore();

    public async findServer(serverId: string): Promise<undefined | FirebaseServer> {
        const serverPath = `servers/${serverId}`;
        const documentContent = await this.getDocumentContent(serverPath);
        return documentContent.exists ? FirebaseServer.convertDocument(documentContent) : undefined;
    }

    public async findConfigurations(serverId: string, object: string): Promise<undefined | Map<string, string>> {
        const configurationPath = `servers/${serverId}/configurations/${object}`;
        const configurationContent = await this.getDocumentContent(configurationPath);
        return configurationContent.exists ? this.documentConverter(configurationContent) : undefined;
    }

    private async getDocumentContent(documentPath: string): Promise<FirebaseFirestore.DocumentSnapshot> {
        const document = this.fireStoreClient.doc(documentPath);
        return await document.get();
    }

    private documentConverter(document: FirebaseFirestore.DocumentSnapshot): Map<string, string> {
        const mappedDocument = new Map();
        const documentData = document.data();
        const documentKeys = documentData ? Object.keys(documentData) : [];
        documentKeys.forEach(key => mappedDocument.set(key, documentData[key]));
        return mappedDocument;
    }

}
