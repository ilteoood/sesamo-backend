import QueryDocumentSnapshot = FirebaseFirestore.QueryDocumentSnapshot;

export class ServerAction {

    accessToken: string;
    event: string;

    static createFromDocument(documentSnapshot: QueryDocumentSnapshot) {
        const serverAction = new ServerAction();
        serverAction.accessToken = documentSnapshot.get('accessToken');
        serverAction.event = documentSnapshot.get('event');
        return serverAction;
    }

}
