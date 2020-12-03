import { ArrowUpSquareFill, ArrowDownSquareFill } from "react-bootstrap-icons";

interface IUpDown {
	upEvent: Function;
	downEvent: Function;
}

export function UpDown({ upEvent, downEvent }: IUpDown) {
	return (
		<div
			style={{
				display: "flex",
				flexDirection: "column",
				marginLeft: 1,
				marginRight: 5,
			}}
		>
			<ArrowUpSquareFill
				onClick={() => upEvent()}
				style={{ marginBottom: 1 }}
			/>
			<ArrowDownSquareFill onClick={() => downEvent()} />
		</div>
	);
}
