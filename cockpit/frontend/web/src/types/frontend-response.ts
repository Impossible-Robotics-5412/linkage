export type FrontendResponse =
	| {
			success: true;
			data: any;
	  }
	| {
			success: false;
			error: Error;
	  };
