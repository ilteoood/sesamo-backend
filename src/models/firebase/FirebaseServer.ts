import DocumentData = FirebaseFirestore.DocumentData;
import QueryDocumentSnapshot = FirebaseFirestore.QueryDocumentSnapshot;
import {ServerAction} from "./ServerAction";

export class FirebaseServer {
    static readonly FIELDS = ["name", "type"];

    name: string;
    type: string;
    allowedDevices: string[] = [];
    actions: Map<String, ServerAction> = new Map();

    static convertServerDocument(documentData: DocumentData): FirebaseServer {
        const documentContent = documentData.data();
        const firebaseServer = new FirebaseServer();
        this.FIELDS.forEach(field => firebaseServer[field] = documentContent[field]);
        return firebaseServer;
    }

    convertConfigurationsDocument(documents: QueryDocumentSnapshot[]) {
        this.allowedDevices = documents.find(document => document.id === "allowedDevices").get('list');
        documents.filter(document => document.id !== "allowedDevices")
            .forEach(document => this.actions[document.id] = ServerAction.createFromDocument(document));
    }
}
